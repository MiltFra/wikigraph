use super::*;
use reqwest;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use thiserror::Error;

/// A struct representing a Wikipedia article with attributes like
/// the URL, related articles and eventually more.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    /// URL of the article; where you'd find it in your web browser.
    pub url: URL,
    /// All the URLs of other articles that are referenced within the article.
    pub references: HashSet<URL>,
}

/// ArticleErr is an enum that contains possible error values that
/// could occur during the creation of a new Article in Article::new.
///
/// Keep in mind that this includes a lot of I/O operation.
#[derive(Error, Debug)]
pub enum ArticleErr {
    #[error("Line ended while parsing URL")]
    UnexpectedEOL,
}

impl Article {
    pub fn new(url: URL) -> Self {
        Article {
            url: url,
            references: HashSet::new(),
        }
    }
    pub fn parse(url: URL, site: String) -> Result<Self, Box<dyn Error>> {
        let mut refs = HashSet::new();
        let lines = site.lines();
        for mut line in lines {
            while !line.is_empty() {
                if line.starts_with("<a href=\"/wiki/") {
                    line = line.strip_prefix(REFERENCE_PREFIX).unwrap_or("");
                    let end;
                    match line.find('"') {
                        Some(i) => end = i,
                        None => {
                            return Err(Box::new(ArticleErr::UnexpectedEOL));
                        }
                    }
                    if let Ok(ref_url) = URL::new(&line[..end]) {
                        refs.insert(ref_url);
                    }
                    line = &line[end..];
                    continue;
                }
                // Strip one character from the left.
                line = line
                    .chars()
                    .next()
                    .map(|c| &line[c.len_utf8()..])
                    .unwrap_or("");
            }
        }
        let mut v: Vec<String> = refs.iter().map(|x| x.to_string()).collect();
        v.sort();
        Ok(Article {
            url: url,
            references: refs,
        })
    }

    pub fn get_url(&self) -> URL {
        self.url.clone()
    }
}

/// A structs to handle requests to look up one or more specific articles,
/// a neighbourhood around and article or even paths between two articles.
/// 
/// No make this more efficient a Collector has a cache and a reqwest client
/// to limit overhead and the number of actual GET requests sent and articles
/// parsed.
pub struct Collector {
    cache: HashMap<URL, Article>,
    processed: usize,
    client: reqwest::Client,
}
#[derive(Error, Debug)]
pub enum CollectionErr {
    #[error("HTTP request failed.")]
    RequestError,
    #[error("Could not find path in given neighbourhood.")]
    PathFindingError,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            cache: HashMap::new(),
            processed: 0,
            client: reqwest::Client::new(),
        }
    }

    /// Takes a single URL and gets the corresponding articles. If this article has
    /// been looked up before (by this particular object) the result is retreived from
    /// a to limit the number of GET requests this program produces.
    pub async fn get(&mut self, url: &URL) -> Result<Article, Box<dyn Error>> {
        self.processed += 1;
        if let Some(a) = self.cache.get(url) {
            return Ok(a.clone());
        }
        let a = self.get_uncached(url).await?;
        self.cache.insert(url.clone(), a.clone());
        Ok(a)
    }

    /// A function to retrieve the HTML for a specific article by creating a HTTP get request.
    /// The text is then parsed and a new Article object is created.
    ///
    /// Errors that can occur are mostly out of the users control as they are either related
    /// to the I/O actions or to the content of the Wikipedia article which might not be possible
    /// to parse. If this happens, the source code needs to be changed.
    async fn get_uncached(&self, url: &URL) -> Result<Article, Box<dyn Error>> {
        let r = self.client.get(&url.to_string()).send().await?;
        let a = Article::parse(url.clone(), r.text().await?)?;
        println!("{}", a.url.to_string());
        Ok(a)
    }

    /// Takes a vector of URLs and gets the corresponding articles. Note that the resulting
    /// Vec<Article> is guranteed to have the results in the same order as the given Vec<URL>.
    ///
    /// This function does not make good use of concurrency as Collector::get is called for
    /// each URL individually and consecutively. That means that each new HTTP request is only
    /// made if the last one has received a response and has been parsed.
    pub async fn get_list_stable(
        &mut self,
        urls: &Vec<URL>,
    ) -> Result<Vec<Article>, Box<dyn Error>> {
        eprint!("Getting list of {} urls... ", urls.len());
        let mut res = Vec::new();
        for x in urls {
            res.push(self.get(x).await?);
        }
        eprintln!("Done");
        Ok(res)
    }

    /// Takes a vector of URLs and gets the corresponding articles. Note that the resulting
    /// Vec<Article> is not guranteed to have the results in the same order as the given Vec<URL>.
    ///
    /// This function does make havy use of concurrency as the futures are obtained from Collector::get
    /// for each and every URL and then they are joined and awaited. This leads to better usage of the
    /// downtime due to I/O operations.
    pub async fn get_list(&mut self, urls: &Vec<URL>) -> Result<Vec<Article>, Box<dyn Error>> {
        eprint!("Getting list of {} urls... ", urls.len());
        self.processed += urls.len();
        let mut ys = Vec::new(); // Articles for all the inputs in urls
        let mut fs = Vec::new(); // futures that have to be run because no values are cached
        let mut xs = Vec::new(); // urls that have to be evaluated with corresponding articles in fs
        for x in urls {
            if let Some(y) = self.cache.get(x) {
                ys.push(y.clone());
            } else {
                fs.push(self.get_uncached(x));
                xs.push(x);
            }
        }
        // We're awaiting all the futures at once to make use of the parallelism that's built in.
        let res = futures::future::join_all(fs).await;
        for r in xs.into_iter().zip(res) {
            match r {
                (x, Ok(y)) => {
                    self.cache.insert(x.clone(), y.clone());
                    ys.push(y);
                }
                (_, Err(e)) => {
                    return Err(e);
                }
            }
        }
        eprintln!("Done");
        Ok(ys)
    }

    /// Gets all the neighbours of up to a given degree.
    /// All values for the depth are valid as degree 0 means no neighbours are actually looked up.
    pub async fn get_neighbourhood(
        &mut self,
        url: &URL,
        depth: u32,
    ) -> Result<Vec<Article>, Box<dyn Error>> {
        let mut ts = HashSet::new(); // "Unhandled URLs"
        let mut ns = HashSet::new(); // Encountered URLs
        ts.insert(url.clone());
        for _ in 1..depth {
            eprintln!(
                "Extending neighbourhood by {} ({} -> {})",
                ts.len(),
                ns.len(),
                ns.len() + ts.len()
            );
            ns.extend(ts.iter().cloned());
            let urls = ts.into_iter().collect();
            let arts = self.get_list(&urls).await?;
            let mut new_ts = HashSet::new();
            for a in arts {
                for u in a.references.iter().cloned() {
                    if ns.insert(u.clone()) {
                        // We only need to fetch this value if we've not seen it before.very
                        new_ts.insert(u);
                    }
                }
            }
            eprintln!("New Ts: {} entries", new_ts.len());
            ts = new_ts;
        }
        self.get_list(&ns.into_iter().collect()).await
    }

    /// Given two URLs to valid Wikipedia articles this allows to find a chain of articles that
    /// connects the two inputs by references.
    pub async fn get_path(&mut self, og: &URL, tg: &URL) -> Result<Vec<Article>, Box<dyn Error>> {
        let mut ts = HashSet::new(); // "Unhandled URLs"
        let mut ns = HashSet::new(); // Encountered URLs
        ts.insert(og.clone());
        while !ts.contains(tg) {
            ns.extend(ts.iter().cloned());
            let arts = self.get_list(&ts.into_iter().collect()).await?;
            let mut new_ts = HashSet::new();
            for a in arts {
                for u in a.references.iter().cloned() {
                    if ns.insert(u.clone()) {
                        new_ts.insert(u);
                    }
                }
            }
            ts = new_ts;
        }
        self.find_path(og, tg, ns.into_iter().collect()).await
    }

    /// Given a neighbourhood (i.e. a set, or rather a Vector, of URLs that are guranteed to contain a path between og and tg)
    /// the path is found. It is extremely important that the given neighbourhood does indeed contain the desired path, otherwise the code may panic.
    ///
    /// # Panics
    ///
    /// 1. If the given set of nodes does not contain a valid path from og to tg.
    /// 2. If either og or tg is not in the given set.
    async fn find_path(
        &mut self,
        og: &URL,
        tg: &URL,
        mut ns: Vec<URL>,
    ) -> Result<Vec<Article>, Box<dyn Error>> {
        ns.sort();
        let l = ns.len();
        let mut adj = vec![false; l * l];
        let mut seen = vec![false; l];
        let og_idx = ns
            .binary_search(&og)
            .expect("Origin for required path is not in given neighbourhood.");
        let tg_idx = ns
            .binary_search(&tg)
            .expect("Target for required path is not in given neighbourhood.");
        let mut q = VecDeque::new();
        q.push_back(og_idx);
        seen[og_idx] = true;
        while !seen[tg_idx] {
            let v = q
                .pop_front()
                .expect("Target could not be visited before exhausting neighbourhood.");
            let a = self.get(&ns[v]).await?;
            for r in a.references {
                if let Ok(k) = ns.binary_search(&r) {
                    adj[l * v + k] = true; // Create edge v -> k
                                           //eprintln!("Created edge {}->{}", ns[v].get_name(), ns[k].get_name());
                    if !seen[k] {
                        // If we've already seen this then we don't need to visit it again.
                        q.push_back(k);
                        seen[k] = true;
                    }
                }
            }
        }
        let mut path = Vec::new();
        let bd = binary_dijkstra(&adj, l, og_idx, tg_idx).unwrap();
        for i in bd {
            path.push(self.get(&ns[i]).await?);
        }
        Ok(path)
    }
}

/// A simplified (but naive) version of Dijkstra's algorithm to find a path in a directed graph
/// given by an adjacency matrix without edge weights.
///
/// The main advantage of these constraints is that the first distance that is determined for any
/// single node is guranteed to be the shortest distance as all the edges have the same length and
/// any other node is at least as many steps away from the origin.
fn binary_dijkstra(adj: &Vec<bool>, l: usize, og: usize, tg: usize) -> Option<Vec<usize>> {
    if og >= l || tg >= l {
        return None;
    }
    if adj.len() != l * l {
        // We require adj to be a "square" matrix.
        return None;
    }
    let mut visited = vec![false; l]; // Whether or not a node has been processed, i.e. visited.
    let mut from = vec![None; l]; // The neighbour the shortest path to a node comes from.
    let mut dist = vec![0; l]; // "Tentative distance" of a node from og (note that it's either infinty, i.e. -1, or the actual distance)
    let mut q = VecDeque::new(); // Queue of vertices to handle
    dist[og] = 0;
    visited[og] = true;
    for n in neighs(adj, l, og) {
        from[n] = Some(og);
        q.push_back(n);
    }
    while dist[tg] < 0 && !q.is_empty() {
        let v = q.pop_front().unwrap();
        let p = from[v].unwrap();
        dist[v] = dist[p] + 1;
        visited[v] = true;
        for n in neighs(adj, l, v) {
            if let None = from[n] {
                from[n] = Some(v);
                q.push_back(n);
            }
        }
    }
    let mut path = Vec::new();
    let mut v = tg;
    while v != og {
        path.push(v);
        v = from[v].unwrap();
    }
    path.reverse();
    Some(path)
}

/// Returns the indices of the neighbours for any node in a graph given by and adjacency matrix.
///
/// This function is potentially unsafe as it does not check the size of the adjacency matrix and
/// may therefore try to access indices out of bounds.
fn neighs(adj: &Vec<bool>, l: usize, v: usize) -> Vec<usize> {
    let mut ns = Vec::new();
    for i in 0..l {
        if adj[l * v + i] {
            ns.push(i);
        }
    }
    ns
}

mod tests {
    // For some reason rust gives warnings that these imports are unneeded. Removing them leads to compile time errors, though.
    use super::{Collector, URL};
    use std::error::Error;

    #[test]
    fn get_is_deterministic() -> Result<(), Box<dyn Error>> {
        let mut runtime = tokio::runtime::Builder::new()
            .basic_scheduler()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        let u = URL::new("https://en.wikipedia.org/wiki/Wikipedia")?;
        let mut c = Collector::new();
        let r = runtime.block_on(c.get(&u))?;
        for _ in 0..100 {
            assert_eq!(runtime.block_on(c.get(&u))?, r);
        }
        Ok(())
    }

    #[test]
    fn get_list_is_deterministic() -> Result<(), Box<dyn Error>> {
        let mut runtime = tokio::runtime::Builder::new()
            .basic_scheduler()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
        let us = vec![
            URL::new("https://en.wikipedia.org/wiki/Wikipedia")?,
            URL::new("https://en.wikipedia.org/wiki/Tree")?,
        ];
        let mut c = Collector::new();
        let r = runtime.block_on(c.get_list(&us))?;
        for _ in 0..100 {
            assert_eq!(runtime.block_on(c.get_list(&us))?, r);
        }
        Ok(())
    }
}

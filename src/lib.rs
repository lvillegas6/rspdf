

mod units;
mod paper_format;
mod page;
use page::{Page};


struct RsPdf {
    pages: Vec<Page>,
}

impl RsPdf {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {


    }
}

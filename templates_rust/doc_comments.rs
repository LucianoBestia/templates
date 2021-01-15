
for function

    /// Searches this `Rejection` for a specific cause.
    ///
    /// A `Rejection` will accumulate causes over a `Filter` chain. This method
    /// can search through them and return the first cause of this type.
    ///
    /// # Example
    ///
    /// ```
    /// #[derive(Debug)]
    /// struct Nope;
    ///
    /// impl warp::reject::Reject for Nope {}
    ///
    /// let reject = warp::reject::custom(Nope);
    ///
    /// if let Some(nope) = reject.find::<Nope>() {
    ///    println!("found it: {:?}", nope);
    /// }
    /// ```
    
    
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // A String has a memory layout of:
        // - pointer to a `Vec<u8>`
        // - length of the string
        // - capacity of the string
        //
        // Each of these values is a `usize`, so the total size of a String is
        // 3 times the size of a `usize`.
        assert_eq!(size_of::<String>(), size_of::<usize>() * 3);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), size_of::<usize>() * 3 * 3);
    }
}

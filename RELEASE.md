# Final notes
In this release, we have restructured the code and introduced benchmarks to give a rough idea of the performance of the different algorithms used in RHC. This will be the final release for now, as I want to focus on other projects and feel that I have achieved the goals that this project set out to accomplish.

Overall, this project has been an amazing learning experience. Starting with little knowledge of Rust, I was able to implement a rather large project consisting of approximately 5000 lines of code. My goal for this project was to limit the use of third-party libraries, in order to challenge myself to become better at Rust and problem-solving in general.

Planning out a larger project can be a daunting task, especially when working alone. In order to stay organized, I broke down the project into smaller tasks and created a schedule for completing each task. This allowed me to focus on one aspect of the project at a time, and ensured that I didn't overlook any important details.

Developing the CLI was a significant challenge, as it required creating an intuitive and user-friendly interface. Initially, I considered using an object-oriented approach to design the CLI, as it would have allowed for a more modular and scalable implementation. However, after some consideration, I ultimately settled on a more procedural design approach that better suited the needs of the project.

By using a procedural design, I was able to create a simpler and more straightforward implementation that was easier to debug and maintain. While an object-oriented design may have been more flexible and modular, it would have introduced additional complexity and overhead that was unnecessary for the scope of the project.

Regardless of the design approach, creating an intuitive and user-friendly interface was crucial for the success of the project. I spent a lot of time researching best practices and experimenting with different designs to ensure that users could easily navigate and use the program. In the end, I believe that the resulting CLI was both functional and easy to use, which is a testament to the effort put into its design and implementation.

Implementing SHA-1 & SHA-2 was a challenging task, as it required a deep understanding of cryptography and hashing algorithms. One of the main difficulties I faced was figuring out how to have both algorithms function in the same way, despite their differences in implementation.

After some research and experimentation, I found that using generics and traits in Rust was an effective way to solve this problem. By defining a trait that specified the behavior of a hash function, I was able to write a generic function that could take any hash function that implemented the trait.

Using generics and traits also made the code more modular and reusable. By abstracting away the details of each specific hash function, I was able to write code that was more generic and could be easily adapted to handle different types of hash functions. This approach also made the code easier to test and debug, as I could write tests that checked the behavior of the hash function rather than the specific implementation details.

Overall, using generics and traits was an effective solution to the challenge of implementing SHA-1 and SHA-2 in a unified way. It allowed me to write code that was more modular, reusable, and easier to test and debug, and it demonstrated the power and flexibility of Rust's type system.

The introduction of threading posed significant challenges for me. Initially, I had no idea how to handle data transfers between threads or how to divide the input data across different threads in a safe and efficient manner. However, after conducting extensive research and debugging, I was able to devise a simple yet effective solution that utilized channels.

By using channels, I was able to establish a safe and efficient communication mechanism between threads. This approach allowed me to divide the input data into smaller chunks and distribute them across different threads for parallel processing. Additionally, I decided to write a central manager that handled the communication between threads, simplifying the code and making it easier to manage.

Although the process of introducing threading was challenging, the resulting performance gains were significant. By leveraging the power of multiple threads, I was able to significantly reduce the overall processing time, making the program much faster and more efficient. Overall, the experience of implementing threading taught me a lot about concurrency and parallel programming, and I feel much more confident in my ability to tackle similar challenges in the future.

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut students = [
        "Basu, Avranil",
        "Carino, Olivia L.",
        "Chandler, Jake M.",
        "Fang, Ruili",
        "Gilliam, Jarod S.",
        "Harsha, Sushruth",
        "Hasan, Mohd T.",
        "He, Xin",
        "Hwang, In Joon",
        "Ingle, Abha V.",
        "Jiang, Ting",
        "Johnston, Evan",
        "Kadam, Nruthya Satish",
        "Kota, Siva Narayana",
        "Kumar, Pranay",
        "Lonarkar, Aishwarya P.",
        "McGarity, Hunter K.",
        "Rai, Shubhangi",
        "Sentell, Matthew",
        "Shi, Jiameng",
        "Shinde, Aditya Prakash",
        "Shingate, Shubham Vasant",
        "Sonar, Abhijeet B.",
        "Talekar, Rutuja Dhananjay",
        "Taraf, Marcos A.",
        "Veeravalli, Sai Nagesh",
        "Wagle, Omkar V.",
        "Xu, Chenqian",
    ];

    let papers_date = [
        ["Is Rust Used Safely by Software Developers?", "03/16/2022"],
        [
            "Sandcrust: Automatic Sandboxing of Unsafe Components in Rust.",
            "03/23/2022",
        ],
        ["Securing unsafe rust programs with XRust.", "03/30/2022"],
        [
            "MirChecker: Detecting Bugs in Rust Programs via Static Analysis.",
            "04/06/2022",
        ],
        ["Isolation in Rust: What is Missing?", "04/06/2022"],
        [
            "SyRust: Automatic Testing of Rust Libraries with Semantic-Aware Program Synthesis.",
            "04/13/2022",
        ],
        [
            "Memory-Safety Challenge Considered Solved? An In-Depth Study with All Rust CVEs.",
            "04/13/2022",
        ],
        [
            "RustBelt: securing the foundations of the rust programming language.",
            "04/20/2022",
        ],
        ["RustBelt Meets Relaxed Memory.", "04/20/2022"],
        ["Translating C to safer Rust.", "04/26/2022"],
        [
            "Towards Memory Safe Enclave Programming with Rust-SGX.",
            "04/26/2022",
        ],
        [
            "RusTEE: Developing Memory-Safe ARM TrustZone Applications.",
            "04/26/2022",
        ],
        ["Understanding memory and thread safety practices and issues in real-world Rust programs.", "04/27/2022"],
        ["Stacked Borrows: An Aliasing Model for Rust.", "04/27/2022"],
    ];

    println!("Total # of Students: {}", students.len());

    let mut rng = thread_rng();
    students.shuffle(&mut rng);

    println!("Paper assignment:\n-------------------------------------------------------------------------------------");

    for i in 0..papers_date.len() {
        println!(
            "{:02}. {:25} & {:25} \n{}\n{}\n-------------------------------------------------------------------------------------",
            i + 1,
            students[i], students[(i+14)%28],
            papers_date[i][0],
            papers_date[i][1]
        );
    }
}

// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
struct Persons<'a> {
    name: Vec<&'a str>,
    title: Vec<&'a str>
}


fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(1)).collect();
    let titles: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(4)).collect();

    let persons = Persons{name: names, title: titles};

    let name_title = persons.name.iter().zip(persons.title.iter());

    for (name, title) in name_title.take(5) {
        println!("name: {:?}, title: {:?}", name, title)
    }

}

// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
    id: u32,
    state: State,
}

impl<State> Luggage<State> {
    fn forward<NextState>(self, state:NextState) -> Luggage<NextState> {
        Luggage {
            id: self.id,
            state: state,
        }
    }
}

struct CheckIn;

impl Luggage<CheckIn> {
    fn new(id:u32) -> Self {
        Luggage {
            id: id,
            state: CheckIn,
        }
    }
    fn forward_to_loading(self) -> Luggage<OnLoading> {
        self.forward(OnLoading)
    }
}

struct OnLoading;
impl Luggage<OnLoading> {
    fn forward_to_offloading(self) -> Luggage<OffLoading> {
        self.forward(OffLoading)
    }
}
struct OffLoading;
impl Luggage<OffLoading> {
    fn forward_to_pickup(self) -> Luggage<AwaitingPickup> {
        self.forward(AwaitingPickup)
    }
}

struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn forward_to_custody(self) -> Luggage<EndCustody> {
        self.forward(EndCustody)
    }
}

struct EndCustody;



fn main() {
    let luggage = Luggage::new(1);
    let luggage_state = luggage.forward_to_loading().forward_to_offloading().forward_to_pickup().forward_to_custody();
    match luggage_state.state {
        EndCustody => println!("Luggage {} is in custody", luggage_state.id),
        _ => println!("Luggage {} is not in custody", luggage_state.id),
    }
}

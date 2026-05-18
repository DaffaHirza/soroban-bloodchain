#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};


#[contracttype]
#[derive(Clone, Debug)]
pub struct Donor {
    id: u64,
    name: String,
    blood_type: String,
    location: String,
    status: String,
    reward_point: u32,
    donor_count: u32,
    qr_code: String,
}


#[contracttype]
#[derive(Clone, Debug)]
pub struct EmergencyRequest {
    id: u64,
    hospital_name: String,
    blood_type: String,
    location: String,
    status: String,
}


const DONOR_DATA: Symbol = symbol_short!("DONOR");
const REQUEST_DATA: Symbol = symbol_short!("REQUEST");


#[contract]
pub struct BloodDonorContract;

#[contractimpl]
impl BloodDonorContract {


    pub fn get_donors(env: Env) -> Vec<Donor> {

        return env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));
    }


    pub fn create_donor(
        env: Env,
        name: String,
        blood_type: String,
        location: String,
    ) -> String {

        let mut donors: Vec<Donor> = env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));

        // QR sederhana
        let qr = String::from_str(&env, "QR-DONOR");

        let donor = Donor {
            id: env.prng().gen::<u64>(),
            name: name,
            blood_type: blood_type,
            location: location,
            status: String::from_str(&env, "Available"),
            reward_point: 0,
            donor_count: 0,
            qr_code: qr,
        };

        donors.push_back(donor);

        env.storage().instance().set(&DONOR_DATA, &donors);

        return String::from_str(
            &env,
            "Donor berhasil ditambahkan"
        );
    }


    pub fn update_status(
        env: Env,
        id: u64,
        new_status: String,
    ) -> String {

        let mut donors: Vec<Donor> = env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..donors.len() {

            let mut donor = donors.get(i).unwrap();

            if donor.id == id {

                donor.status = new_status;

                donors.set(i, donor);

                env.storage().instance().set(&DONOR_DATA, &donors);

                return String::from_str(
                    &env,
                    "Status donor berhasil diupdate"
                );
            }
        }

        return String::from_str(
            &env,
            "Donor tidak ditemukan"
        );
    }


    pub fn delete_donor(env: Env, id: u64) -> String {

        let mut donors: Vec<Donor> = env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..donors.len() {

            if donors.get(i).unwrap().id == id {

                donors.remove(i);

                env.storage().instance().set(&DONOR_DATA, &donors);

                return String::from_str(
                    &env,
                    "Donor berhasil dihapus"
                );
            }
        }

        return String::from_str(
            &env,
            "Donor tidak ditemukan"
        );
    }


    pub fn search_by_blood(
        env: Env,
        blood_type: String,
    ) -> Vec<Donor> {

        let donors: Vec<Donor> = env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..donors.len() {

            let donor = donors.get(i).unwrap();

            if donor.blood_type == blood_type {

                result.push_back(donor);
            }
        }

        return result;
    }


    pub fn add_reward(
        env: Env,
        id: u64,
        point: u32,
    ) -> String {

        let mut donors: Vec<Donor> = env
            .storage()
            .instance()
            .get(&DONOR_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..donors.len() {

            let mut donor = donors.get(i).unwrap();

            if donor.id == id {

                donor.reward_point += point;
                donor.donor_count += 1;

                donors.set(i, donor);

                env.storage().instance().set(&DONOR_DATA, &donors);

                return String::from_str(
                    &env,
                    "Reward donor berhasil ditambahkan"
                );
            }
        }

        return String::from_str(
            &env,
            "Donor tidak ditemukan"
        );
    }


    pub fn create_emergency_request(
        env: Env,
        hospital_name: String,
        blood_type: String,
        location: String,
    ) -> String {

        let mut requests: Vec<EmergencyRequest> = env
            .storage()
            .instance()
            .get(&REQUEST_DATA)
            .unwrap_or(Vec::new(&env));

        let request = EmergencyRequest {
            id: env.prng().gen::<u64>(),
            hospital_name: hospital_name,
            blood_type: blood_type,
            location: location,
            status: String::from_str(&env, "Emergency"),
        };

        requests.push_back(request);

        env.storage().instance().set(&REQUEST_DATA, &requests);

        return String::from_str(
            &env,
            "Emergency request berhasil dibuat"
        );
    }



    pub fn get_emergency_requests(
        env: Env
    ) -> Vec<EmergencyRequest> {

        return env
            .storage()
            .instance()
            .get(&REQUEST_DATA)
            .unwrap_or(Vec::new(&env));
    }
}

mod test;
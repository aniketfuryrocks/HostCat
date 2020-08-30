use users::get_current_uid;

pub fn check_privileges() {
    if get_current_uid() != 0 {
        panic!("Not root, run with sudo or as root")
    }
}
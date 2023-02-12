wit_bindgen_guest_rust::generate!("host");

struct MyHost;

impl Host for MyHost {
    fn run(){
        prints("hello world!");
    }
}  

export_host!(MyHost);

use std::collections::HashMap;

fn main() {

    let mut mydb:HashMap<String, String> = HashMap::new();

    mydb.insert("Ram".to_string(),"12".to_string());
    mydb.insert("Adil".to_string(),"28".to_string());
    mydb.insert("Ziya".to_string(),"16".to_string());

    if mydb.contains_key("Ziya"){
        println!("{}","Ziya is in the HashMap")
    }

    //println!("{:?}",mydb.get("Bkob"));

    match mydb.get("Ziya"){
        Some(abc)=>println!("Age: {}", abc),
        None => println!("Ziya is NOT found")
    };

    mydb.remove("Adil");
    match mydb.get("Adil"){
        Some(abc)=>println!("Age: {}", abc),
        None => println!("Adil is NOT found")
    };

    for (k,v) in mydb{
        if k =="Ziya"{
            println!("Found Ziya, her age is {:?}\n",v)
        }
    };
    


}
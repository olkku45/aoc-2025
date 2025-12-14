/*
1. ota file stringiin
2. splittaa tää string pilkun mukaan niin että jokainen ID range on
alkio arrayssä
3. splittaa jokainen alkio tässä arrayssä pareihin viivan mukaan
4. parsi nää parit numeroiksi
5. tee loop jossa numerot ovat loopin range
6. kato loopissa koostuuko nykyinen numero jostain numerosarjasta joka
toistuu kahdesti
    6.1 muunna numero takas stringiks ja splittaa se keskeltä kahtia (jos pystyy)
    6.2 kato ovatko nämä kaksi numeroa samat
7. jos numerot ovat samat, ota kyseinen numero ja lisää se johonkin counteriin
*/

fn do_thing() {
    let file = std::fs::read_to_string("../input.txt").unwrap();
    let split_by_comma: Vec<&str> = file.split(',').collect();

    let mut counter = 0;
    
    for range in split_by_comma {
        let pair: Vec<&str> = range.split('-').collect();
        let clean1 = pair[0].replace("\n", "");
        let clean2 = pair[1].replace("\n", "");

        let low: i64 = clean1.parse().unwrap();
        let high: i64 = clean2.parse().unwrap();

        for n in low..=high {
            let as_str = n.to_string();

            if as_str.len() % 2 != 0 { continue; }
            
            let slices = as_str.split_at(as_str.len() / 2);

            let cmp1: i32 = slices.0.parse().unwrap();
            let cmp2: i32 = slices.1.parse().unwrap();

            if cmp1 == cmp2 {
                counter += n;
            }
        }
    }

    println!("{}", counter);
}

fn main() {
    do_thing();
}

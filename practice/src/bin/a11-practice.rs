// Topic: Ownership

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;

    // display_light(dull);
    // 위 함수에서 dull이 display_light로 이동하여 소멸되었기 때문에 실행 불가.
    // display_light(dull);

    // &을 추가하여 대여를 하여야 정상적으로 실행
    display_light(&dull);
    display_light(&dull);
}

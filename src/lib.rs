/// This function calculates if the (argument) year is bissextile
pub fn bissextile(ano: u16) -> bool {
    let by_four: bool = ano % 4 == 0 ;
    let by_hundred: bool = ano % 100 == 0 ;
    let by_four_hundred: bool  = ano % 400 == 0 ;

    match by_four {
        true => {
            match by_hundred {
                true => { match by_four_hundred {
                    true => { true },
                    false => { false }
                } },
                false => { true }
            }
        },
        false => { false }
    }
}

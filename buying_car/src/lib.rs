pub fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut trigger = false;
    let mut percent = perc;
    let mut old_price: f64 = old as f64;
    let mut new_price: f64 = new as f64;
    let mut month = 0;

    loop {
        trigger = !trigger;

        if (old_price + ((saving * month) as f64)) < new_price {
            old_price = old_price - old_price / 100.0 * percent;
            new_price = new_price - new_price / 100.0 * percent;
        } else {
            return (
                month,
                (old_price - new_price + (saving * month) as f64).round() as i32,
            );
        }

        if trigger {
            percent += 0.5;
        }
        month += 1;
    }
}

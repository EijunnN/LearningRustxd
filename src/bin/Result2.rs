#[derive(Debug)]
enum WendyAction {
    Caminar,
    Parar,
    Correr,
}

fn get_action(action: &str) -> Result<WendyAction, String> {
    match action {
        "caminar" => Ok(WendyAction::Caminar),
        "parar" => Ok(WendyAction::Parar),
        "correr" => Ok(WendyAction::Correr),
        _ => Err("Wendy no puede realizar esta accion".to_lowercase()),
    }
}

fn print_action(action: &WendyAction) {
    println!("La accion que realizaste es : {:?}", action);
}

fn pick_action(action: &str) -> Result<(), String> {
    let accion = get_action(action)?;
    print_action(&accion);
    Ok(())
}

fn main() {
    let accion = pick_action("caerse");
    println!("Realizaste esto {:?}", accion)
}

mod r#README

use std::collections::HashMap;



#[derive(Debug)]
//Detalhes da reserva
struct DetalhesReserva{
    id: u32,
    nome_hotel: String,
    numero_quarto: u32,
    data_checkIn: String,
    data_checkOut: String,
}

//  Criação HashMap
struct ReservaHotel {
    reservas:  HashMap<u32, DetalhesReserva>
}

impl ReservaHotel {
    //Adicionando instância
    pub fn new() -> Self {
        Self {
            reservas: HashMap::new(),
        }
    }

    // Adicionar reserva
     pub fn adiciona(&mut self, id: u32, nome_hotel: String, numero_quarto: u32, data_checkin: String, data_checkout: String,) -> Option<DetalhesReserva> {
        let codigo_reserva = id;
        self.reservas.insert(codigo_reserva, DetalhesReserva {
            id,
            nome_hotel,
            numero_quarto,
            data_checkIn,
            data_checkOut,
        })
    }


    //Recuperando reservas
    pub fn recupera_reservas(&self, id_reserva: u32) -> Option<&DetalhesReserva> {
        self.reservas.get(&id_reserva)
    }

}



fn main(){

    println!("Sistema de reserva de hotéis");



    let reserva1 = DetalhesReserva{
        id: 1,
        nome_hotel: String::from("Hotel A"),
        numero_quarto: 12,
        data_checkIn: String::from("12/12/24"),
        data_checkOut: String::from("14/12/24"),
    };



}
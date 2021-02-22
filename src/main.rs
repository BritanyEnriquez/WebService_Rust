#![feature(proc_macro_hygiene, decl_macro)]

use std::usize;

#[macro_use] extern crate rocket;

#[get("/saludo")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/resolver/<numeroPal>")]
fn resolver(numeroPal: String) -> String {
    
    let tokens:Vec<&str>= numeroPal.split(".").collect();
    let mut numero = tokens[0].parse::<usize>().unwrap();
    let mut decimales = tokens[1].parse::<usize>().unwrap();

    let mut Unidad=numero%10;
    let mut Decena=(numero%100)/10;
    let mut Centena=(numero/100)%10;

    let mut UnidadMillon = (numero/1000)%10;
    let mut DecenaMillon = (numero/10000)%10;
    let mut CentenaMillon = numero/100000;

    let mut UnidadDecimales=decimales%10;
    let mut DecenaDecimales=(decimales%100)/10;
    let mut CentenaDecimales=(decimales/100)%10;

    let mut vectorU = vec!["","Uno","Dos","Tres","Cuatro","Cinco","Seis","Siete","Ocho","Nueve"];
    let mut vectorD =vec!["","Diez","Veint","Treinta","Cuarenta","Cincuenta","Sesenta","Setenta","Ochenta","Noventa"];
    let mut vectorDE =vec!["","Once","Doce","Trece","Catorce","Quince"];
    let mut vectorC = vec!["","Cien","Docientos","Trecientos","Cuatrocientos","Quinientos","Seiscientos","Setecientos","Ochocientos","Novecientos"];
    
    if numero == 0
    {
        format!("El numero es Cero")
    } 
    //UNIDAD DECENA CENTENA NO DECIMALES
    else if UnidadMillon==0 && DecenaMillon==0 && CentenaMillon==0 && UnidadDecimales==0 && DecenaDecimales==0 && CentenaDecimales==0
    {
        if Unidad == 0
        {
            if Decena == 2
            {
                format!("El numero escrito es {} {}e",vectorC[Centena],vectorD[Decena])
            }
            else
            {
                format!("El numero escrito es {} {}",vectorC[Centena],vectorD[Decena])
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            format!("El numero escrito es {}", vectorU[Unidad])
        }
        else if Unidad ==1 && Decena == 1
        {
            format!("El numero escrito es {} {}",vectorC[Centena],vectorDE[1])
        }
        else if Unidad ==2 && Decena == 1
        {
            format!("El numero escrito es {} {} ",vectorC[Centena],vectorDE[2])
        }
        else if Unidad ==3 && Decena == 1
        {
            format!("El numero escrito es {} {} ",vectorC[Centena],vectorDE[3])
        }
        else if Unidad ==4 && Decena == 1
        {
            format!("El numero escrito es {} {} ",vectorC[Centena],vectorDE[4])
        }
        else if Unidad ==5 && Decena == 1
        {
            format!("El numero escrito es {} {} ",vectorC[Centena],vectorDE[5])
        }
        else if  Decena == 2
        {
            format!("El numero escrito es {} {}i{}",vectorC[Centena],vectorD[2],vectorU[Unidad])
        }
        else if  Centena == 1
        {
            format!("El numero escrito es {}to {} y {}",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }
        else
        {
            format!("El numero escrito es {} {} y {} ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }

    }
    //UNIDAD DECENA CENTENA MILESIMAS NO DECIMALES
    else if UnidadDecimales==0 && DecenaDecimales==0 && CentenaDecimales==0//UNIDAD-DECENA-CENTENA MILESIMA
    {

        if Unidad == 0
        {
            if Decena == 2
            {
                format!("El numero escrito es {} {} {}e",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena])
            }
            else
            {
                format!("El numero escrito es {} {} {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena])
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            format!("El numero escrito es {} {}",mil(UnidadMillon,DecenaMillon,CentenaMillon), vectorU[Unidad])
        }
        else if Unidad ==1 && Decena == 1
        {
            format!("El numero escrito es {} {} {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[1])
        }
        else if Unidad ==2 && Decena == 1
        {
            format!("El numero escrito es {} {} {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[2])
        }
        else if Unidad ==3 && Decena == 1
        {
            format!("El numero escrito es {} {} {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[3])
        }
        else if Unidad ==4 && Decena == 1
        {
            format!("El numero escrito es {} {} {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[4])
        }
        else if Unidad ==5 && Decena == 1
        {
            format!("El numero escrito es {} {} {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[5])
        }
        else if  Decena == 2
        {
            format!("El numero escrito es {} {} {}i{}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[2],vectorU[Unidad])
        }
        else if  Centena == 1
        {
            format!("El numero escrito es {} {}to {} y {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }
        else
        {
            format!("El numero escrito es {} {} {} y {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }

    }
    else if UnidadMillon==0 && DecenaMillon==0 && CentenaMillon==0//CON DECIMALES
    {
        if Unidad == 0
        {
            if Decena == 2
            {
                format!("El numero escrito es {} {}e coma {}",vectorC[Centena],vectorD[Decena],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
            }
            else
            {
                format!("El numero escrito es {} {} coma {}",vectorC[Centena],vectorD[Decena],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            format!("El numero escrito es {} coma {}", vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==1 && Decena == 1
        {
            format!("El numero escrito es {} {} coma {}",vectorC[Centena],vectorDE[1], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==2 && Decena == 1
        {
            format!("El numero escrito es {} {} coma {} ",vectorC[Centena],vectorDE[2],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==3 && Decena == 1
        {
            format!("El numero escrito es {} {} coma {}",vectorC[Centena],vectorDE[3], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==4 && Decena == 1
        {
            format!("El numero escrito es {} {} coma {}",vectorC[Centena],vectorDE[4], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==5 && Decena == 1
        {
            format!("El numero escrito es {} {} coma {}",vectorC[Centena],vectorDE[5], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if  Decena == 2
        {
            format!("El numero escrito es {} {}i{} coma {}",vectorC[Centena],vectorD[2],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if  Centena == 1
        {
            format!("El numero escrito es {}to {} y {} coma {}",vectorC[Centena],vectorD[Decena],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else
        {
            format!("El numero escrito es {} {} y {} coma {}",vectorC[Centena],vectorD[Decena],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
    }
    else //CON DECIMALES
    {
        if Unidad == 0
        {
            if Decena == 2
            {
                format!("El numero escrito es {} {} {}e coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
            }
            else
            {
                format!("El numero escrito es {} {} {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            format!("El numero escrito es {} {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon), vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==1 && Decena == 1
        {
            format!("El numero escrito es {} {} {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[1],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==2 && Decena == 1
        {
            format!("El numero escrito es {} {} {} coma {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[2], decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==3 && Decena == 1
        {
            format!("El numero escrito es {} {} {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[3],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==4 && Decena == 1
        {
            format!("El numero escrito es {} {} {} coma {} ",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[4],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if Unidad ==5 && Decena == 1
        {
            format!("El numero escrito es {} {} {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorDE[5],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if  Decena == 2
        {
            format!("El numero escrito es {} {} {}i{} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[2],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else if  Centena == 1
        {
            format!("El numero escrito es {} {}to {} y {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
        else
        {
            format!("El numero escrito es {} {} {} y {} coma {}",mil(UnidadMillon,DecenaMillon,CentenaMillon),vectorC[Centena],vectorD[Decena],vectorU[Unidad],decimas(UnidadDecimales,DecenaDecimales,CentenaDecimales))
        }
    }

}

fn mil(Unidad : usize, Decena : usize, Centena : usize) -> String
{

    let mut vectorU = vec!["","Uno","Dos","Tres","Cuatro","Cinco","Seis","Siete","Ocho","Nueve"];
    let mut vectorD =vec!["","Diez","Veint","Treinta","Cuarenta","Cincuenta","Sesenta","Setenta","Ochenta","Noventa"];
    let mut vectorDE =vec!["","Once","Doce","Trece","Catorce","Quince"];
    let mut vectorC = vec!["","Cien","Docientos","Trecientos","Cuatrocientos","Quinientos","Seiscientos","Setecientos","Ochocientos","Novecientos"];
    

    if Unidad == 0
        {
            if Decena == 2
            {
                format!(" {} {}e mil ",vectorC[Centena],vectorD[Decena])
            }
            else
            {
                //let palabra=String::from("El numero escrito es {} {} mil ",vectorC[Centena],vectorD[Decena]);
                format!(" {} {}",vectorC[Centena],vectorD[Decena])
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            if Unidad==1
            {
                format!(" mil ")
            }
            else
            {
                format!(" {} mil ", vectorU[Unidad])
            }
            
        }
        else if Unidad ==1 && Decena == 1
        {
            format!(" {} {} mil ",vectorC[Centena],vectorDE[1])
        }
        else if Unidad ==2 && Decena == 1
        {
            format!(" {} {} mil ",vectorC[Centena],vectorDE[2])
        }
        else if Unidad ==3 && Decena == 1
        {
            format!(" {} {} mil ",vectorC[Centena],vectorDE[3])
        }
        else if Unidad ==4 && Decena == 1
        {
            format!(" {} {} mil ",vectorC[Centena],vectorDE[4])
        }
        else if Unidad ==5 && Decena == 1
        {
            format!(" {} {} mil ",vectorC[Centena],vectorDE[5])
        }
        else if  Decena == 2
        {
            format!(" {} {}i{} mil ",vectorC[Centena],vectorD[2],vectorU[Unidad])
        }
        else if  Centena == 1
        {
            format!(" {}to {} y {} mil ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }
        else
        {
            format!(" {} {} y {} mil ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }

}

fn decimas(Unidad : usize, Decena : usize, Centena : usize) -> String
{

    let mut vectorU = vec!["","Uno","Dos","Tres","Cuatro","Cinco","Seis","Siete","Ocho","Nueve"];
    let mut vectorD =vec!["","Diez","Veint","Treinta","Cuarenta","Cincuenta","Sesenta","Setenta","Ochenta","Noventa"];
    let mut vectorDE =vec!["","Once","Doce","Trece","Catorce","Quince"];
    let mut vectorC = vec!["","Cien","Docientos","Trecientos","Cuatrocientos","Quinientos","Seiscientos","Setecientos","Ochocientos","Novecientos"];
    

    if Unidad == 0
        {
            if Decena == 2 && Centena ==0
            {
                format!(" {} {}e Centesimas ",vectorC[Centena],vectorD[Decena])
            }
            else if Decena ==2
            {
                format!(" {} {}e Milesimas ",vectorC[Centena],vectorD[Decena])
            }
            else if Centena==0
            {
                //let palabra=String::from("El numero escrito es {} {} mil ",vectorC[Centena],vectorD[Decena]);
                format!(" {} {} Centesimas",vectorC[Centena],vectorD[Decena])
            }
            else
            {
                format!(" {} {} Milesimas",vectorC[Centena],vectorD[Decena])
            }
            
        }
        else if Centena == 0 && Decena == 0
        {
            if Unidad==1
            {
                format!(" una decima ")
            }
            else
            {
                format!(" {} decimas ", vectorU[Unidad])
            }
            
        }
        else if Unidad ==1 && Decena == 1
        {
            if Centena ==0
            {
                format!(" {} {} Centesimas ",vectorC[Centena],vectorDE[1])
            }
            else
            {
                format!(" {} {} Milesimas ",vectorC[Centena],vectorDE[1])
            }
            
        }
        else if Unidad ==2 && Decena == 1
        {
            if Centena ==0
            {
                format!(" {} {} Centesimas ",vectorC[Centena],vectorDE[2])
            }
            else
            {
                format!(" {} {} Milesimas ",vectorC[Centena],vectorDE[2])
            }
            
        }
        else if Unidad ==3 && Decena == 1
        {
            if Centena ==0
            {
                format!(" {} {} Centesimas ",vectorC[Centena],vectorDE[3])
            }
            else
            {
                format!(" {} {} Milesimas ",vectorC[Centena],vectorDE[3])
            }
            
        }
        else if Unidad ==4 && Decena == 1
        {
            if Centena ==0
            {
                format!(" {} {} Centesimas ",vectorC[Centena],vectorDE[4])
            }
            else
            {
                format!(" {} {} Milesimas ",vectorC[Centena],vectorDE[4])
            }
            
        }
        else if Unidad ==5 && Decena == 1
        {
            if Centena ==0
            {
                format!(" {} {} Centesimas ",vectorC[Centena],vectorDE[5])
            }
            else
            {
                format!(" {} {} Milesimas ",vectorC[Centena],vectorDE[5])
            }
            
        }
        else if  Decena == 2
        {
            if Centena ==0
            {
                format!(" {} {}i{} Centesimas ",vectorC[Centena],vectorD[2],vectorU[Unidad])
            }
            else
            {
                format!(" {} {}i{} Milesimas ",vectorC[Centena],vectorD[2],vectorU[Unidad])
            }
            
        }
        else if  Centena == 1
        {
            
            format!(" {}to {} y {} Milesimas ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
        }
        else
        {
            if Centena ==0
            {
                format!(" {} {} y {} Centesimas ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
            }
            else
            {
                format!(" {} {} y {} Milesimas ",vectorC[Centena],vectorD[Decena],vectorU[Unidad])
            }
            
        }

}
fn main() {
    rocket::ignite().mount("/", routes![index,resolver]).launch();
}
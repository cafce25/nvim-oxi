use nvim_oxi::{self as oxi, api, lua, print, Dictionary, Function, Object};
use oxi::conversion;
use oxi::serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Car {
    manufacturer: CarManufacturer,

    miles: u32,

    #[serde(default)]
    problem: Option<CarProblem>,

    #[serde(default = "yep")]
    works: bool,
}

fn yep() -> bool {
    true
}

#[derive(Copy, Clone, Serialize, Deserialize)]
enum CarManufacturer {
    Nikola,
    Tesla,
    Volkswagen,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CarProblem {
    DoesntMove,
    KillsPedestrians,
    Pollutes,
}

impl TryFrom<Object> for Car {
    type Error = conversion::Error;
    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl TryFrom<Car> for Object {
    type Error = conversion::Error;
    fn try_from(car: Car) -> Result<Object, Self::Error> {
        car.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for Car {
    unsafe fn pop(
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::try_from(obj)
            .map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for Car {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        Car::try_from(self)
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

fn fix(mut car: Car) -> oxi::Result<Car> {
    if car.works {
        return Ok(car);
    }

    if car.problem.is_none() {
        api::err_writeln("Well, what's the issue?");
        return Ok(car);
    }

    use CarManufacturer::*;
    use CarProblem::*;

    match (car.manufacturer, car.problem.unwrap()) {
        (Nikola, DoesntMove) => print!("Try going downhill"),
        (Tesla, KillsPedestrians) => print!("Hands on the wheel!!"),
        (Volkswagen, Pollutes) => print!("Software update?"),
        _ => {},
    }

    car.works = true;
    car.problem = None;

    Ok(car)
}

#[nvim_oxi::module]
fn mechanic() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([("fix", Function::from_fn(fix))]))
}

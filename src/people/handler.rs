use diesel::result::Error;
use people::{InsertablePerson, Person};
use people::connection::DbConn;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::status;
use rocket_contrib::Json;

#[get("/")]
fn all(connection: DbConn) -> Result<Json<Vec<Person>>, Failure> {
    Person::all(&connection)
        .map(|people| Json(people))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn get(id: i32, connection: DbConn) -> Result<Json<Person>, Failure> {
    Person::get(id, &connection)
        .map(|person| Json(person))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    })

//    match error {
//        Error::NotFound => Failure(Status::NotFound),
//        _ => Failure(Status::InternalServerError)
//    }

//    if error == Error::NotFound {
//        Failure(Status::NotFound)
//    } else {
//        Failure(Status::InternalServerError)
//    }

//    let code = if error == Error::NotFound {
//        Status::NotFound
//    } else {
//        Status::InternalServerError
//    };
//    Failure(code)

//            if error == Error::NotFound {
//                return Failure(Status::NotFound);
//            }
//            Failure(Status::InternalServerError)
}

#[post("/", format = "application/json", data = "<person>")]
fn post(person: Json<Person>,
        connection: DbConn,
) -> Result<status::Created<Json<Person>>, Failure> {
    InsertablePerson::insert(person.into_inner(), &connection)
        .map(|person| person_created(person))
        .map_err(|_| Failure(Status::InternalServerError))
}

fn person_created(person: Person) -> status::Created<Json<Person>> {
    status::Created(
        format!("localhost:8080/people/{}", person.id).to_string(),
        Some(Json(person)))
}

#[put("/<id>", format = "application/json", data = "<person>")]
fn put(id: i32, person: Json<Person>, connection: DbConn) -> Result<Json<Person>, Failure> {
    Person::update(id, person.into_inner(), &connection)
        .map(|person| Json(person))
        .map_err(|_| Failure(Status::InternalServerError))
}

#[delete("/<id>")]
fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Failure> {
    match Person::get(id, &connection) {
        Ok(_) => Person::delete(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
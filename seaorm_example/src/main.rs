use seaorm_example::sea_orm_example;

fn main() {
    //let db_example: DatabaseConnection = Database::connect("postgres://username:password@host/database?currentSchema=my_schema").await?;

    println!("Sum is :{}", sea_orm_example::add(1, 2));
    sea_orm_example::hellworld();
}
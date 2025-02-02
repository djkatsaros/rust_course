/*
 * Bank project 
 *
 * implment account fucntionality
 * Account with 
 *  balance     : integer
 *  acct number : unsigned integer
 *  holder      : string
 */

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
// needs to store accounts
    accounts: Vec<Account>, 
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
    
    fn deposit(&mut self, amount: i32) -> i32  {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] } //implicit return 
    }

    fn add_account(&mut self, account: Account) { //pass mutable reference to self
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|acct| acct.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|acct| acct.summary())
            .collect::<Vec<String>>()
    }
}



fn main() {

    let mut bank = Bank::new();
    let mut account = Account::new(1,String::from("me"));

    account.deposit(500);
    account.withdraw(250);
    
    println!("{}", account.summary());

    bank.add_account(account);
    
    println!("{:#?}", bank.total_balance());
    println!("{:#?}", bank.summary());     
}


/*
 * Ownership, Borrowing, and Lifetimes:
 *  Goal of rust ownrshp: limit the ways you can reference and change data
 *  -> reduce number of bugs and make code easier to understand 
 *  possilbe rules
 *  -> use references: multiple things can refer to a value at the same time, but the referce
 *  ensures the value is read-only
 *  -> value can only be updated when there are no read-only (referneces ) to it
 *  ** these rules are implemented in rust!! **
 *  ** avoid unexpected updates to objects.
 * 
 * Rules
 *  1. every value is 'owned' by a single var at a time
 *  2. reassigning a value to another binding (var) moves the value, the old binding cant be used
 *  to access the value anymore. 
 *  
 *  ## Borrowing ##
 *  Rules
 *    3. you can create many read-only refs to a value that exist at the same time
 *    4. you can't move a value while a ref to the value exists
 *    5. can make a mutable ref to a value only if there are no read-only refs currently in
 *       use, only one mutable ref to a val can exist at a time
 *    6. You cannot mutate a value throug hthe owner when any ref (mut or immut) to the val exists
 *    7. Soe types of values are copied instead of moves (numbers, bools, chars, arrays/tuples with
 *       copyable elements). 
 *
 *  ## Lifetimes ##
 *  refers to how long an owner/reference exists
 *      generic lifetimes/ lifetime annotations: syntax added to clarify relationships b/w
 *      lifetimes
 *  Rules
 *    8. When an owner goes out of scope, the value owned by it is dropped
 *    9. There can't be references to a value when its owner goes out of scope
 *    10. References to a value cant outlive the value they refer to.
 *   
 *  Rough guidelines for fcn arg types:
 *      Need to store arg somewhere?             -> facor taking ownership (receive a value)
 *      Need to do a calculation with the value? -> favor receiving read=only reference
 *      Need to change the value in some way?    -> favor receiving a mutable reference
 *
 */

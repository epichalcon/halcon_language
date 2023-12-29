print("hola mundo");

let a = true;

if (a) {
    print("sip");
}
else {
    print("nop");
}

let fibonacci = fun(x) {
    if (x == 0) {
        0
    } else {
        if (x == 1) {
            1
        } else {
            fibonacci(x - 1) + fibonacci(x - 2);
        }
    }
};

print(fibonacci(10));

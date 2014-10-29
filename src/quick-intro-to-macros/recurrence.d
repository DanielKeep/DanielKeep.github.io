import std.range : recurrence, take;
import std.stdio : writeln;

void main() {
    foreach(e ; take(recurrence!"a[n-1] + a[n-2]"(1uL, 1), 10)) {
        out_int(e);
    }
}

void out_int(ulong e) {
    writeln(e);
}

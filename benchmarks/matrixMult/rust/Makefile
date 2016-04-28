sniper_roi.o: src/sniper_roi.c
	gcc -c src/sniper_roi.c -o src/sniper_roi.o -fPIC

libsniper_roi.a: sniper_roi.o
	ar -rcs src/libsniper_roi.a src/sniper_roi.o

rust: libsniper_roi.a src/main.rs
	./cargo build


clean:
	rm -f src/libsniper_roi.a src/sniper_roi.o
	./cargo clean

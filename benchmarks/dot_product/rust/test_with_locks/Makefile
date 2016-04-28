all: lsniper_roi
	rustc dotproduct.rs

sniper_roi.o:
	gcc -c sniper_roi.c -o sniper_roi.o -fPIC

lsniper_roi: sniper_roi.o
	ar -rcs libsniper_roi.a sniper_roi.o

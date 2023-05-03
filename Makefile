build:
	gcc ./*.c -lallegro -lallegro_primitives
format:
	clang-format ./*.c --style=Chromium -i
#include <stdio.h>
#include <string.h>
#include <stddef.h>

#define MEMORY_SIZE 30000

struct Brain {
  unsigned char memory[MEMORY_SIZE];
  size_t whereis;
};

struct Brain brainsucker(void) {
  struct Brain b;
  memset(b.memory, 0, MEMORY_SIZE);
  b.whereis = 0;
  return b;
}


void suck_in(struct Brain* br) {
  br->memory[br->whereis] = (br->memory[br->whereis] + 1) % 256;
}
void suck_de(struct Brain* br) {
  br->memory[br->whereis] = (br->memory[br->whereis] - 1) % 256;
}
void suck_move_right(struct Brain* br) {
  if (br->whereis < MEMORY_SIZE - 1) {
    br->whereis++;
  }
}
void suck_move_left(struct Brain* br) {
  if (br->whereis > 0) {
    br->whereis--;
  }
}
unsigned char get_current(struct Brain* br) {
  return br->memory[br->whereis];
}
void set_data(struct Brain* br, unsigned char data) {
  br->memory[br->whereis] = data;
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    printf("Usage: %s <brainfck_file>", argv[0]);
    return 1;
  }

  char* path = argv[1];
  FILE* file = fopen(path, "r");
  struct Brain brainsuck = brainsucker();

  if (file == NULL) {
    printf("Error: Could not open file\n");
    return 1;
  }

  unsigned char code[100000] = {0};
  unsigned int code_len = 0;
  char line_buf[256] = {0};

  while (fgets(line_buf, 256, file) != NULL) {
    int j = 0;
    while (j < 256 && line_buf[j] != 0 && code_len < 99999) {
      code[code_len] = (unsigned char)line_buf[j];
      code_len++;
      j++;
    }
  }

  fclose(file);

  int i = 0;
  while (i < code_len) {
    switch (code[i]) {
    case '+':
      suck_in(&brainsuck);
      break;
    case '-':
      suck_de(&brainsuck);
      break;
    case '>':
      suck_move_right(&brainsuck);
      break;
    case '<':
      suck_move_left(&brainsuck);
      break;
    case '.':
      unsigned char c = get_current(&brainsuck);
      printf("%c", c);
      break;
    case ',':
      char inpchar;
      scanf("%c", &inpchar);
      set_data(&brainsuck, inpchar);
      break;
    case '[':
      if (get_current(&brainsuck) == 0) {
	int bracket_count = 1;
	while (bracket_count > 0 && i < code_len - 1) {
	  i++;
	  switch (code[i]) {
	  case '[':
	    bracket_count++;
	    break;
	  case ']':
	    bracket_count--;
	    break;
	  }
	}
      }
      break;
    case ']':
      if (get_current(&brainsuck) != 0) {
	int bracket_count = 1;
	while (bracket_count > 0 && i > 0) {
	  i--;
	  switch (code[i]) {
	  case ']':
	    bracket_count++;
	    break;
	  case '[':
	    bracket_count--;
	    break;
	  }
	}
      }
      break;
    }
    i++;
  }
  
  return 0;
}

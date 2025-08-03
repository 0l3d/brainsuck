#include <stddef.h>
#include <stdio.h>
#include <string.h>

#define MEMORY_SIZE 30000

// MAIN COMPILER
void create_fasm(char *code, FILE *out) {
  int loop_id = 0;
  int loop_stack[1000];
  int stack_ptr = 0;

  fprintf(out, "format ELF64 executable 3\n"
               "segment readable writeable\n"
               "tape rb 30000\n"
               "segment readable executable\n"
               "entry _start\n"
               "_start:\n"
               "lea rsi, [tape]\n");

  for (int i = 0; code[i] != 0; i++) {
    switch (code[i]) {
    case '>':
      fprintf(out, "inc rsi \n");
      break;
    case '<':
      fprintf(out, "dec rsi\n");
      break;
    case '+':
      fprintf(out, "inc byte [rsi]\n");
      break;
    case '-':
      fprintf(out, "dec byte [rsi]\n");
      break;
    case '.':
      fprintf(out, "mov rax, 1\n"
                   "mov rdi, 1\n"
                   "mov rdx, 1\n"
                   "syscall\n");
      break;
    case ',':
      fprintf(out, "mov rax, 0\n"
                   "mov rdi, 0\n"
                   "mov rdx, 1\n"
                   "syscall\n");
      break;

    case '[': {
      int current_id = loop_id++;
      loop_stack[stack_ptr++] = current_id;
      fprintf(out, "loop_start_%d_open:\n", current_id);
      fprintf(out, "cmp byte [rsi], 0\n");
      fprintf(out, "je loop_end_%d_close\n", current_id);
      break;
    }
    case ']': {
      if (stack_ptr <= 0) {
        fprintf(stderr, "Unmatched ] %d\n", i);
        return;
      }
      int current_id = loop_stack[--stack_ptr];
      fprintf(out, "cmp byte [rsi], 0\n");
      fprintf(out, "jne loop_start_%d_open\n", current_id);
      fprintf(out, "loop_end_%d_close:\n", current_id);
      break;
    }
    }
  }

  if (stack_ptr != 0) {
    fprintf(stderr, "Unmatched [\n");
    return;
  }
  fprintf(out, "mov rax, 60\n"
               "xor rdi, rdi\n"
               "syscall\n");
}

// MAIN INTERPRETER
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

void suck_in(struct Brain *br) {
  br->memory[br->whereis] = (br->memory[br->whereis] + 1) % 256;
}
void suck_de(struct Brain *br) {
  br->memory[br->whereis] = (br->memory[br->whereis] - 1) % 256;
}
void suck_move_right(struct Brain *br) {
  if (br->whereis < MEMORY_SIZE - 1) {
    br->whereis++;
    {
    }
  }
}
void suck_move_left(struct Brain *br) {
  if (br->whereis > 0) {
    br->whereis--;
  }
}
unsigned char get_current(struct Brain *br) { return br->memory[br->whereis]; }
void set_data(struct Brain *br, unsigned char data) {
  br->memory[br->whereis] = data;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    printf("Usage: %s <brainfck_file> | [-c] compile code to fasm", argv[0]);
    return 1;
  }

  int compile = 1;

  FILE *outfasm = fopen("compiled.brainsuck.asm", "w");
  if (outfasm == NULL) {
    perror("create fasm open file errro");
    return -1;
  }

  char *path = argv[1];
  FILE *file = fopen(path, "r");
  struct Brain brainsuck = brainsucker();

  if (file == NULL) {
    printf("openfile error\n");
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

  if (argc > 2) {
    if (strcmp(argv[2], "-c") == 0) {
      create_fasm(code, outfasm);
    }
  } else {
    remove("compiled.brainsuck.asm");
  }
  fclose(outfasm);
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

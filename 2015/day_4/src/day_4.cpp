#include <atomic>
#include <iomanip>
#include <iostream>
#include <openssl/evp.h>
#include <sstream>
#include <thread>
#include <vector>

const std::string input = "iwrupvqb";

std::atomic<bool> found(false);
std::atomic<int> resultSeed(-1);

std::string calculateHash(const std::string &input) {
  EVP_MD_CTX *mdctx = EVP_MD_CTX_new();    // Crear contexto MD
  unsigned char md_value[EVP_MAX_MD_SIZE]; // Buffer para el hash
  unsigned int md_len;                     // Longitud del hash

  // Inicializar contexto y especificar MD5
  EVP_DigestInit_ex(mdctx, EVP_md5(), NULL);
  // Pasar los datos a procesar
  EVP_DigestUpdate(mdctx, input.c_str(), input.size());
  // Finalizar y obtener el hash
  EVP_DigestFinal_ex(mdctx, md_value, &md_len);
  EVP_MD_CTX_free(mdctx); // Liberar contexto

  // Convertir el hash binario a una cadena hexadecimal
  std::ostringstream oss;
  for (unsigned int i = 0; i < md_len; ++i) {
    oss << std::hex << std::setw(2) << std::setfill('0') << (int) md_value[i];
  }
  return oss.str();
}

void findHash(const std::string &secret, int totalOfZeroes, const std::string &zeroes, int start, int step) {
  int seed = start;

  while (!found) {
    std::string input = secret + std::to_string(seed);
    std::string hash = calculateHash(input);

    if (hash.substr(0, totalOfZeroes) == zeroes) {
      found = true;
      resultSeed = seed;
      return;
    }

    seed += step;
  }
}

void part(int totalOfZeroes, const std::string &zeroes, int numThreads) {
  std::vector<std::thread> threads;

  for (int i = 0; i < numThreads; ++i) {
    threads.emplace_back(findHash, input, totalOfZeroes, zeroes, i, numThreads);
  }

  for (auto &t : threads) {
    t.join();
  }

  std::cout << "Result for " << totalOfZeroes << " zeroes: Seed = " << resultSeed << std::endl;
  found = false;
  resultSeed = -1;
}

int main() {
  int numThreads = std::thread::hardware_concurrency();

  std::cout << "Number of threads: " << numThreads << std::endl;

  std::cout << "Running with " << numThreads << " threads." << std::endl;

  // Part 1: 5 zeroes
  part(5, "00000", numThreads);

  // Part 2: 6 zeroes
  part(6, "000000", numThreads);

  return 0;
}

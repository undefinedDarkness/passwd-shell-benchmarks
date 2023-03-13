#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

using namespace std;

int main() {
  // Open the password file for reading.
  ifstream passwd_file("passwd");

  // Define a hash table to store the number of instances of each shell.
  unordered_map<string, int> shell_counts;

  // Read the file line by line and count the instances of each shell.
  string line;
  while (getline(passwd_file, line)) {
    // Find the last colon-separated field (the login shell).
    size_t last_colon = line.rfind(':');
    string shell = line.substr(last_colon + 1);

    // Increment the count of this shell.
    ++shell_counts[shell];
  }

  // Print out the counts for each shell in three columns.
  int column_width = 25;
  for (const auto &[k, v] : shell_counts) {
    cout << k << ": " << v << endl;
  }

  return 0;
}

#!/usr/bin/env crystal


# Define the size of chunks to split the file into
CHUNK_SIZE = 1000

# Create an empty hash to store shell and its count
shellcnt = Hash(String, Int32).new(0)

# Split the file into chunks and process each chunk concurrently
File.open("passwd") do |file|
    chunks = file.each_line.each_slice(CHUNK_SIZE).to_a

    fibers = chunks.map do |chunk|
        # Process each line in the chunk and update the shell count
        spawn do
            chunk.each do |line|
                # Extract the shell from the last field of the line
                shell = line.split(':').last.chomp

                # Increment the count for the given shell in the hash
                shellcnt[shell] += 1
            end
        end
    end

    # Wait for all fibers to complete
    Fiber.yield 
end

# Print the shell count in formatted output
shellcnt.each do |shell, count|
    printf("%-20s%-8s%-d\n", shell, ":", count)
end

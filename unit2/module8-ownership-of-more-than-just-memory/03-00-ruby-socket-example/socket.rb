require 'socket'

def open_socket_for_five_seconds
  s = TCPServer.new '127.0.0.1', 5000
  sleep 5
end

open_socket_for_five_seconds
puts "back in main"
sleep 5

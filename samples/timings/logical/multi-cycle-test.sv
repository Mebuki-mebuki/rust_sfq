`include "all.v"
`include "modules.v"

module top;
  // Inputs
  reg a, clk;
  // Outputs
  wire y;

  reg __cycle;
  integer cycles;
  MultiCyclePath dut (.*);

  // Simulation cycles
  initial begin
    __cycle = 0;
    cycles  = 0;
    forever begin
      #1;
      __cycle = 0;
      #1;
      __cycle = 1;
      cycles  = cycles + 1;
    end
  end

  // Input patterns
  localparam N = 12;
  assign clk = 1;
  logic [N-1:0] patterns = 'b0110_1111_0000;

  integer i;
  initial begin
    a = 0;
    for (i = 0; i < N; i++) begin
      @(posedge __cycle);
      a <= patterns[N-1-i];
    end
    $finish;
  end

  initial begin
    $dumpfile("multi-cycle-test.vcd");
    $dumpvars(0, top);
  end
endmodule

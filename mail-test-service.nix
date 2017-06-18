{ config, lib, pkgs, ... }:

with lib;

let

  cfg = config.services.mailtest;

in

{

  ###### interface

  options = {

    services.mailtest = {

      enable = mkOption {
        default = false;
        description = "Whether to enable the rust mail test service";
      };

    };
  };


  ###### implementation

  config = mkIf cfg.enable {

    environment.systemPackages = [ pkgs.ssmtp ];

    systemd.services.mailtest = {

      description = "Mail test";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      environment = {
        RUST_LOG="debug";
      };
      serviceConfig = {
        PIDFile = "/var/db/mailtest/mailtest.pid";
        ExecStart = "/var/db/mailtest/bin/rust-nixos-sendmail";
      };

    };

  };

}

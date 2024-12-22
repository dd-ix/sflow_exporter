{ config, pkgs, lib, ... }:

let
  cfg = config.services.sflow-exporter;
in
{
  options.services.sflow-exporter = {
    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.sflow-exporter;
      defaultText = lib.literalExpression "pkgs.sflow-exporter";
      description = lib.mdDoc "Which sflow_exporzer derivation to use.";
    };
    enable = lib.mkEnableOption "sflow_exporter";
    listen = {
      sflow = {
        addr = lib.mkOption {
          type = lib.types.str;
          description = lib.mdDoc "The ip address the sflow listener should be listening on.";
          default = "::";
        };
        port = lib.mkOption {
          type = lib.types.port;
          description = lib.mdDoc "The port the sflow listener should be listening on.";
          default = 6343;
        };
      };
      metrics = {
        addr = lib.mkOption {
          type = lib.types.str;
          description = lib.mdDoc "The ip address the metrics listener should be listening on.";
          default = "::";
        };
        port = lib.mkOption {
          type = lib.types.port;
          description = lib.mdDoc "The port the metrics listener should be listening on.";
          default = 9144;
        };
      };
    };
    metaPath = lib.mkOption {
      type = lib.types.str;
      description = lib.mdDoc "The path where the meta configuration file is located.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    systemd.services.sflow-exporter = {
      description = "sflow_exporter";

      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        ExecStart = "${lib.getExe cfg.package} listen";
        DynamicUser = true;
        User = "sflow_exporter";

        Environment =
          let
            sflowAddr = cfg.listen.sflow.addr;
            metricsAddr = cfg.listen.metrics.addr;
          in
          [
            "SFLOW_EXPORTER_SFlOW_LISTEN_ADDR=${if (lib.hasInfix ":" sflowAddr) then "[${sflowAddr}]" else sflowAddr}:${toString cfg.listen.sflow.port}"
            "SFLOW_EXPORTER_METRICS_LISTEN_ADDR=${if (lib.hasInfix ":" metricsAddr) then "[${metricsAddr}]" else metricsAddr}:${toString cfg.listen.metrics.port}"
            "SFLOW_EXPORTER_META=${cfg.metaPath}"
          ];
      };
    };
  };
}


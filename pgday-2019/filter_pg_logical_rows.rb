require 'fluent/plugin/filter'

module Fluent::Plugin
  class PostgresLogicalRowsFilter < Filter
    Fluent::Plugin.register_filter('pg_logical_rows', self)

    def filter_stream(tag, es)
      new_es = Fluent::MultiEventStream.new
      es.each {|time, record|
        record["change"].each {|change|
          target = {}
          target["event_ts"] = record["timestamp"][0,19]
          target["receive_ts"] = time
          target["xid"] = record["xid"]
          target["kind"] = change["kind"]
          target["schema_name"] = change["schema"]
          target["table_name"] = change["table"]

          if not change["columnnames"].nil?
            target["column_names"] = join_list(change["columnnames"])
            target["column_types"] = join_list(change["columntypes"])
            target["column_values"] = join_list(change["columnvalues"])
          end

          if not change["oldkeys"].nil?
            target["old_key_names"] = join_list(change["oldkeys"]["keynames"])
            target["old_key_types"] = join_list(change["oldkeys"]["keytypes"])
            target["old_key_values"] = join_list(change["oldkeys"]["keyvalues"])
          end

          new_es.add(time, target)
        }
      }
      new_es
    end

    private

    def join_list(values)
      values.map{|x| "\"" + x.to_s + "\""}.join(" ")
    end
  end
end

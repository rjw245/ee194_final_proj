clear

[FileName,PathName,~] = uigetfile('.sqlite3');
dbid = mksqlite('open',strcat(PathName,FileName));

values = mksqlite(dbid,'select prefixname, objectname, metricname, core, value from "values" INNER JOIN "prefixes" ON prefixes.prefixid="values".prefixid INNER JOIN names ON names.nameid="values".nameid');

%Only look at test data once sim stopped
values = values(cellfun(@(x) strcmp(x,'stop'), {values.prefixname}));

objectnames = unique(extractfield(values,'objectname'));

set(0,'DefaultFigureWindowStyle','docked')
for object=objectnames
    object_fig = figure('WindowStyle','normal','Name',char(object));
    object_tabgroup = uitabgroup(object_fig);
    ObjectData = values(cellfun(@(x) strcmp(x,object), {values.objectname}));
    metricnames = unique(extractfield(ObjectData,'metricname'));
    for metric=metricnames
        thistab = uitab(object_tabgroup, 'Title', char(metric)); % build iith tab
        MetricData = ObjectData(cellfun(@(x) strcmp(x,metric), {ObjectData.metricname}));
        cores = extractfield(MetricData,'core');
        yvals = extractfield(MetricData,'value');
        cur_axes = axes('Parent',thistab); % somewhere to plot
        bar(cur_axes,cores,yvals);
        title([metric, ' of ', object], 'Interpreter', 'none');
        xlabel('# Core');
        ylabel('Value');
    end
end

%CoreData = values(cellfun(@(x) strcmp(x,'core'), {values.objectname}));

%L1DData = values(cellfun(@(x) strcmp(x,'L1-D'), {values.objectname}));
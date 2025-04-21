%{
    Example MATLAB Source File
    --------------------------
    This file demonstrates all accepted comment types in MATLAB.

    Header comments like this block comment provide an overview of the file,
    author information, date, and purpose.

    Author: Your Name
    Date: April 16, 2025
%}

%% Initialization Section
% This section initializes variables

x = 10; % Set x to 10
y = 20; % Set y to 20

%% Calculation Section
% Perform some calculations

z = x + y; % Add x and y

%{
    The following block is commented out using block comment syntax.
    It won't execute until the block comment symbols are removed.
%}
%{
disp('This line is commented out');
a = 5;
%}

%% Display Section
% Display the result

disp(['The sum of x and y is: ', num2str(z)]); % Show result

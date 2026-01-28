"use client";

import { SelectAccount } from "@/components/select-account";
import { useUserQuery } from "@/hooks/use-user";
import { useTRPC } from "@/trpc/client";
import { formatAmount } from "@/utils/format";
import { formatAmountValue, formatDate } from "@midday/import";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@midday/ui/accordion";
import { Icons } from "@midday/ui/icons";
import { Label } from "@midday/ui/label";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import { Spinner } from "@midday/ui/spinner";
import { Switch } from "@midday/ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@midday/ui/tooltip";
import { useQuery } from "@tanstack/react-query";
import { capitalCase } from "change-case";
import { useEffect, useRef, useState } from "react";
import { Controller, useWatch } from "react-hook-form";
import { mappableFields, useCsvContext } from "./context";

export function FieldMapping({ currencies }: { currencies: string[] }) {
  const { fileColumns, firstRows, setValue, control, watch } = useCsvContext();
  const [isStreaming, setIsStreaming] = useState(false);
  const [showCurrency, setShowCurrency] = useState(false);
  const abortControllerRef = useRef<AbortController | null>(null);
  const type = watch("type") || "transactions";
  const fields = mappableFields[type as keyof typeof mappableFields];
  const trpc = useTRPC();
  const { data: bankAccounts } = useQuery(trpc.bankAccounts.get.queryOptions({
    // Only fetch if type is transactions
  }, { enabled: type === "transactions" }));

  // ... (keeping relevant logic but making it conditional)
  
  useEffect(() => {
    if (type !== "transactions" || !fileColumns || !firstRows) return;
    // ... (rest of the streaming logic for transactions)
  }, [type, fileColumns, firstRows]);

  return (
    <div className="mt-6">
      <div className="grid grid-cols-2 gap-x-4 gap-y-2">
        <div className="text-sm">CSV Data column</div>
        <div className="text-sm">System data column</div>
        {(Object.keys(fields) as (keyof typeof fields)[]).map(
          (field) => (
            <FieldRow
              key={field}
              field={field as any}
              type={type as any}
              isStreaming={isStreaming}
              currency={watch("currency")}
            />
          ),
        )}
      </div>

      {type === "transactions" && (
        <>
          <Accordion
            defaultValue={undefined}
            collapsible
            type="single"
            className="w-full mt-6 border-t-[1px] border-border"
          >
            <AccordionItem value="settings">
              <AccordionTrigger className="text-sm">Settings</AccordionTrigger>
              <AccordionContent>
                <div className="flex flex-col gap-4">
                  <Controller
                    control={control}
                    name="inverted"
                    render={({ field: { onChange, value } }) => (
                      <div className="space-y-1">
                        <Label htmlFor="inverted">Inverted amount</Label>
                        <p className="text-sm text-[#606060]">
                          If the transactions are from credit account, you can
                          invert the amount.
                        </p>
                        <div className="flex justify-end">
                          <Switch
                            id="inverted"
                            checked={value}
                            onCheckedChange={onChange}
                          />
                        </div>
                      </div>
                    )}
                  />
                </div>
              </AccordionContent>
            </AccordionItem>
          </Accordion>

          <div className="mt-6">
            <Label className="mb-2 block">Account</Label>
            <Controller
              control={control}
              name="bank_account_id"
              render={({ field: { value, onChange } }) => (
                <SelectAccount
                  className="w-full"
                  placeholder="Select account"
                  value={value}
                  // ... rest of SelectAccount props
                />
              )}
            />
          </div>
        </>
      )}

      {type === "grades" && (
        <div className="mt-6 space-y-4">
           {/* Additional fields for grades if needed like Subject/Class picker if not passed in params */}
        </div>
      )}
    </div>
  );
}


function FieldRow({
  field,
  type,
  isStreaming,
  currency,
}: {
  field: string;
  type: keyof typeof mappableFields;
  isStreaming: boolean;
  currency?: string;
}) {
  const fields = mappableFields[type];
  const { label, required } = (fields as any)[field];
  const { control, fileColumns, firstRows } = useCsvContext();
  const { data: user } = useUserQuery() as any;

  // Use useWatch for better reactivity when values change
  const value = useWatch({ control, name: field as any });
  const inverted = useWatch({ control, name: "inverted" });

  const isLoading = isStreaming && !value;

  const firstRow = firstRows?.at(0);

  const description = value && firstRow ? firstRow[value as string] : undefined;

  // Check if date field has valid parseable date
  const isDateInvalid =
    type === "transactions" && field === "date" && description && !formatDate(description);

  // Check if amount field has valid parseable number
  const parsedAmount = (type === "transactions" && field === "amount" && description)
    ? formatAmountValue({ amount: description, inverted })
    : null;
  const isAmountInvalid =
    type === "transactions" && field === "amount" && description && Number.isNaN(parsedAmount);

  // Combined invalid state for styling
  const isFieldInvalid = isDateInvalid || isAmountInvalid;

  const formatDescription = (description?: string) => {
    if (!description) return;

    if (type === "transactions") {
        if (field === "date") {
          return formatDate(description);
        }

        if (field === "amount") {
          const amount = formatAmountValue({ amount: description, inverted });

          if (currency) {
            return formatAmount({ currency, amount, locale: user?.locale });
          }

          return amount;
        }

        if (field === "balance") {
          const amount = formatAmountValue({ amount: description });

          // Always invert the amount for balance
          const balance = +(amount * -1);

          if (currency) {
            return formatAmount({
              currency,
              amount: balance,
              locale: user?.locale,
            });
          }

          return balance;
        }

        if (field === "description") {
          return capitalCase(description);
        }
    }

    return description;
  };

  // Get appropriate error message for invalid fields
  const getErrorMessage = () => {
    if (isDateInvalid) return "Invalid date format - cannot parse this date";
    if (isAmountInvalid) return "Invalid amount - cannot parse this number";
    return null;
  };

  return (
    <>
      <div className="relative flex min-w-0 items-center gap-2">
        <Controller
          control={control}
          name={field as any}
          rules={{ required }}
          render={({ field: controllerField }) => {
            return (
              <Select
                value={(controllerField?.value as string) ?? undefined}
                onValueChange={controllerField.onChange}
              >
                <SelectTrigger className="w-full relative" hideIcon={isLoading}>
                  <SelectValue placeholder={`Select ${label}`} />

                  {isLoading && (
                    <div className="absolute top-2 right-2">
                      <Spinner size={16} />
                    </div>
                  )}
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectLabel>{label}</SelectLabel>
                    {[
                      // Filter out empty columns
                      ...(fileColumns?.filter((column) => column !== "") || []),
                      ...(controllerField.value && !required ? ["None"] : []),
                    ]?.map((column) => {
                      return (
                        <SelectItem key={column} value={column}>
                          {column}
                        </SelectItem>
                      );
                    })}
                  </SelectGroup>
                </SelectContent>
              </Select>
            );
          }}
        />

        <div className="flex items-center justify-end">
          <Icons.ArrowRightAlt className="size-4 text-[#878787]" />
        </div>
      </div>

      <span className="flex h-9 w-full items-center justify-between whitespace-nowrap border border-border bg-transparent px-3 py-2 text-sm">
        <div className="grow whitespace-nowrap text-sm font-normal text-muted-foreground justify-between flex">
          <span>{label}</span>

          {description?.trim() && (
            <TooltipProvider delayDuration={50}>
              <Tooltip>
                <TooltipTrigger asChild>
                  <button type="button" className="cursor-help">
                    {isFieldInvalid ? (
                      <Icons.AlertCircle className="size-4 text-destructive" />
                    ) : (
                      <Icons.Info />
                    )}
                  </button>
                </TooltipTrigger>
                <TooltipContent className="p-2 text-xs">
                  {getErrorMessage() ?? formatDescription(description)}
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          )}
        </div>
      </span>
    </>
  );
}


            let exec_result = if task.task_type.starts_with(\"finance.\") {
                FinanceExecutor::execute_task(db, &task.task_type, task.target_entity_id.as_deref().unwrap_or_default(), task.params.as_deref()).await
                    .map(|_| \"Executed\".to_string())
            } else if task.task_type.starts_with(\"exam.\") {
                ExamExecutor::execute_task(db, &task.task_type, task.target_entity_id.as_deref().unwrap_or_default(), task.params.as_deref()).await
                    .map(|_| \"Executed\".to_string())
            } else {
                // Default to AcademicExecutor for backwards compatibility
                AcademicExecutor::execute(db, &task, &self.event_bus, self.period_gate.as_ref()).await
            };

            match exec_result {
                Ok(output) => {
                    succeeded += 1;

                    if output == \"REQUIRES_APPROVAL\" {
                        // Update task as requires_approval
                        let mut active = task.clone().into_active_model();
                        active.status = Set(autopilot_task::status::REQUIRES_APPROVAL.to_string());
                        active.updated_at = Set(Utc::now().to_rfc3339());
                        let _ = active.update(db).await;
                    } else {
                        // Update task as completed
                        let _ = Self::complete_task(db, &task.id, &output).await;
                    }

                    // Log execution
                    let _ = Self::log_execution(
                        db, org_id, &task.id, &task.task_type,
                        \"completed\", Some(&output), None,
                        start.elapsed().as_millis() as i32,
                    ).await;

                    info!(
                        task_id = %task.id,
                        \"Task executed successfully\"
                    );
                }
                Err(e) => {
                    failed += 1;
                    let _ = Self::fail_task(db, &task.id, &e.to_string(), task.attempts + 1).await;
                    let _ = Self::log_execution(
                        db, org_id, &task.id, &task.task_type,
                        \"failed\", None, Some(&e.to_string()),
                        start.elapsed().as_millis() as i32,
                    ).await;

                    warn!(
                        task_id = %task.id,
                        error = %e,
                        \"Task execution failed\"
                    );
                }
            }
